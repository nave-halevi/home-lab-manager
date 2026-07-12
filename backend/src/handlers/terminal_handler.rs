use axum::{
    extract::{
        ws::{
            Message,
            WebSocket,
            WebSocketUpgrade,
        },
        Path,
        State,
    },
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};
use futures_util::{
    sink::SinkExt,
    stream::StreamExt,
};
use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    models::status::{
        EnvironmentStatus,
        InstanceStatus,
    },
    repositories::{
        environment_repo,
        instance_repo,
    },
    utils::ssh_manager,
};

pub async fn ws_terminal_handler(
    State(pool): State<PgPool>,
    ws: WebSocketUpgrade,
    Path(environment_id): Path<Uuid>,
) -> Response {
    println!(
        "[Terminal] WebSocket connection requested for environment '{}'.",
        environment_id
    );

    let environment = match environment_repo::find_environment_by_id(
        &pool,
        environment_id,
    )
    .await
    {
        Ok(Some(environment)) => environment,

        Ok(None) => {
            eprintln!(
                "[Terminal] Environment '{}' was not found.",
                environment_id
            );

            return (
                StatusCode::NOT_FOUND,
                "Environment not found",
            )
                .into_response();
        }

        Err(error) => {
            eprintln!(
                "[Terminal] Failed to retrieve environment '{}': {}",
                environment_id,
                error
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve environment",
            )
                .into_response();
        }
    };

    if environment.status
        != EnvironmentStatus::Running.as_str()
    {
        eprintln!(
            "[Terminal] Environment '{}' is not running. Current status: '{}'.",
            environment_id,
            environment.status
        );

        return (
            StatusCode::CONFLICT,
            "Environment is not running",
        )
            .into_response();
    }

    let instance = match instance_repo::find_by_environment_id(
        &pool,
        environment_id,
    )
    .await
    {
        Ok(Some(instance)) => instance,

        Ok(None) => {
            eprintln!(
                "[Terminal] No instance was found for environment '{}'.",
                environment_id
            );

            return (
                StatusCode::NOT_FOUND,
                "No instance found for environment",
            )
                .into_response();
        }

        Err(error) => {
            eprintln!(
                "[Terminal] Failed to retrieve instance for environment '{}': {}",
                environment_id,
                error
            );

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve instance",
            )
                .into_response();
        }
    };

    if instance.status
        != InstanceStatus::Running.as_str()
    {
        eprintln!(
            "[Terminal] Instance '{}' is not running. Current status: '{}'.",
            instance.id,
            instance.status
        );

        return (
            StatusCode::CONFLICT,
            "Instance is not running",
        )
            .into_response();
    }

    let ssh_port = match instance.ssh_port {
        Some(port) => match u16::try_from(port) {
            Ok(port) => port,

            Err(_) => {
                eprintln!(
                    "[Terminal] Invalid SSH port '{}' stored for instance '{}'.",
                    port,
                    instance.id
                );

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid SSH port stored for instance",
                )
                    .into_response();
            }
        },

        None => {
            eprintln!(
                "[Terminal] Instance '{}' does not have an SSH port.",
                instance.id
            );

            return (
                StatusCode::CONFLICT,
                "Instance does not have an SSH endpoint",
            )
                .into_response();
        }
    };

    println!(
        "[Terminal] Environment '{}' resolved to instance '{}' on SSH port '{}'.",
        environment_id,
        instance.id,
        ssh_port
    );

    ws.on_upgrade(move |socket| {
        handle_socket(
            socket,
            environment_id,
            ssh_port,
        )
    })
}

async fn handle_socket(
    socket: WebSocket,
    environment_id: Uuid,
    ssh_port: u16,
) {
    println!(
        "[Terminal] WebSocket upgraded for environment '{}'.",
        environment_id
    );

    let (
        mut websocket_sender,
        mut websocket_receiver,
    ) = socket.split();

    /*
        Browser input is forwarded to the blocking SSH thread.

        SSH output is forwarded back to the asynchronous
        WebSocket sender.
    */
    let (input_sender, input_receiver) =
        mpsc::channel::<String>(100);

    let (output_sender, mut output_receiver) =
        mpsc::channel::<String>(100);

    /*
        Temporary credentials.

        These should later move into Scenario configuration,
        environment secrets, or a secure credential store.
    */
    const SSH_USERNAME: &str = "kali-target";
    const SSH_PASSWORD: &str = "1213";

    let ssh_task = tokio::task::spawn_blocking(move || {
        ssh_manager::connect_and_bridge(
            ssh_port,
            SSH_USERNAME,
            SSH_PASSWORD,
            input_receiver,
            output_sender,
        )
    });

    let mut websocket_send_task =
        tokio::spawn(async move {
            while let Some(output) =
                output_receiver.recv().await
            {
                if websocket_sender
                    .send(Message::Text(output))
                    .await
                    .is_err()
                {
                    println!(
                        "[Terminal] Browser disconnected while sending output."
                    );

                    break;
                }
            }

            println!(
                "[Terminal] WebSocket output task finished."
            );
        });

    let mut websocket_receive_task =
        tokio::spawn(async move {
            while let Some(message_result) =
                websocket_receiver.next().await
            {
                let message = match message_result {
                    Ok(message) => message,

                    Err(error) => {
                        eprintln!(
                            "[Terminal] WebSocket receive error: {}",
                            error
                        );

                        break;
                    }
                };

                match message {
                    Message::Text(input) => {
                        if input_sender
                            .send(input)
                            .await
                            .is_err()
                        {
                            eprintln!(
                                "[Terminal] SSH input channel was closed."
                            );

                            break;
                        }
                    }

                    Message::Binary(_) => {
                        println!(
                            "[Terminal] Ignoring unsupported binary WebSocket message."
                        );
                    }

                    Message::Ping(_) => {
                        /*
                            Axum's WebSocket implementation handles
                            WebSocket control frames internally.
                        */
                    }

                    Message::Pong(_) => {}

                    Message::Close(_) => {
                        println!(
                            "[Terminal] Browser requested WebSocket closure."
                        );

                        break;
                    }
                }
            }

            println!(
                "[Terminal] WebSocket input task finished."
            );
        });

    tokio::select! {
        _ = &mut websocket_send_task => {
            websocket_receive_task.abort();
        }

        _ = &mut websocket_receive_task => {
            websocket_send_task.abort();
        }

        ssh_result = ssh_task => {
            match ssh_result {
                Ok(Ok(())) => {
                    println!(
                        "[Terminal] SSH bridge closed normally for environment '{}'.",
                        environment_id
                    );
                }

                Ok(Err(error)) => {
                    eprintln!(
                        "[Terminal] SSH bridge failed for environment '{}': {}",
                        environment_id,
                        error
                    );
                }

                Err(error) => {
                    eprintln!(
                        "[Terminal] SSH blocking task failed for environment '{}': {}",
                        environment_id,
                        error
                    );
                }
            }

            websocket_send_task.abort();
            websocket_receive_task.abort();
        }
    }

    println!(
        "[Terminal] Session cleaned up for environment '{}'.",
        environment_id
    );
}
