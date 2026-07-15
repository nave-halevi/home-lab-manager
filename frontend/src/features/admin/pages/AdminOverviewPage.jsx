import AdminHeader from "../components/AdminHeader";
import AdminStatCard from "../components/AdminStatCard";
import useAdminOverview from "../hooks/useAdminOverview";

export default function AdminOverviewPage() {
  const { data, loading, error } = useAdminOverview();

  if (loading) return <State text="Loading overview..." />;
  if (error) return <State text={error} error />;

  const statistics = data.statistics;
  const cards = [
    ["Total Users", statistics.total_users],
    ["Active Users", statistics.active_users],
    ["Disabled Users", statistics.disabled_users],
    ["Admin Users", statistics.admin_users],
    ["Total Courses", statistics.total_courses],
    ["Published Courses", statistics.published_courses],
    ["Total Scenarios", statistics.total_scenarios],
    ["Active Scenarios", statistics.active_scenarios],
    ["Running Labs", statistics.running_labs],
    ["Completed Tasks", statistics.completed_tasks],
    ["Submitted Flags", statistics.submitted_flags],
  ];

  return (
    <Page>
      <AdminHeader
        title="Admin Overview"
        description="Live platform statistics and operational status."
      />
      <div className="grid gap-5 sm:grid-cols-2 xl:grid-cols-4">
        {cards.map(([label, value]) => (
          <AdminStatCard key={label} label={label} value={value} />
        ))}
      </div>
      <section className="mt-8 rounded-xl border border-zinc-800 bg-zinc-900 p-5">
        <h2 className="text-lg font-semibold">Recent Admin Activity</h2>
        {!data.recent_activity?.length ? (
          <p className="mt-3 text-sm text-zinc-500">No Admin activity has been recorded yet.</p>
        ) : (
          <div className="mt-4 space-y-3">
            {data.recent_activity.map((item) => (
              <div key={item.id} className="flex flex-wrap items-center justify-between gap-3 border-b border-zinc-800 pb-3 last:border-b-0 last:pb-0">
                <div>
                  <p className="font-medium">{item.action}</p>
                  <p className="text-xs text-zinc-500">
                    {item.admin_user_name || "Unknown Admin"} · {item.entity_type}
                  </p>
                </div>
                <span className="text-xs text-zinc-500">{new Date(item.created_at).toLocaleString()}</span>
              </div>
            ))}
          </div>
        )}
      </section>
    </Page>
  );
}

export function Page({ children }) {
  return <main className="p-6 text-white lg:p-10">{children}</main>;
}

function State({ text, error }) {
  return <div className={`p-10 ${error ? "text-red-400" : "text-zinc-400"}`}>{text}</div>;
}
