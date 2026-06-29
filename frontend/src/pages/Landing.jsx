import { useNavigate } from "react-router-dom";

import Button from "../components/ui/Button";

export default function Landing() {
  const navigate = useNavigate();

  return (
    <main className="min-h-screen bg-zinc-950 text-white">
      {/* Hero */}
      <section className="relative overflow-hidden">
        {/* Background Glow */}
        <div className="absolute left-1/2 top-40 h-96 w-96 -translate-x-1/2 rounded-full bg-red-600/10 blur-3xl" />

        <div className="relative mx-auto flex min-h-[88vh] max-w-7xl items-center px-6">
          {/* Left */}
          <div className="flex-1">
            <p className="mb-5 text-sm font-bold uppercase tracking-[0.35em] text-red-500">
              Cyber Security Training Platform
            </p>

            <h1 className="text-6xl font-black leading-tight">
              Learn.
              <br />
              Practice.
              <br />
              Break.
              <br />
              Understand.
            </h1>

            <p className="mt-8 max-w-xl text-lg leading-8 text-zinc-400">
              Train inside realistic Linux environments, solve real security
              challenges, improve Bash skills, learn networking, web
              exploitation, privilege escalation and Capture The Flag
              techniques.
            </p>

            <div className="mt-10 flex gap-4">
              <Button onClick={() => navigate("/register")}>Get Started</Button>

              <Button variant="secondary" onClick={() => navigate("/login")}>
                Login
              </Button>
            </div>
          </div>

          {/* Right */}
          <div className="hidden flex-1 justify-end lg:flex">
            <div className="w-[480px] rounded-xl border border-zinc-800 bg-zinc-900 p-6 shadow-2xl">
              <div className="mb-6 flex gap-2">
                <div className="h-3 w-3 rounded-full bg-red-500" />
                <div className="h-3 w-3 rounded-full bg-yellow-500" />
                <div className="h-3 w-3 rounded-full bg-green-500" />
              </div>

              <pre className="overflow-x-auto text-left text-sm leading-7 text-zinc-300">
                {`$ ssh student@cyberrange

Connected.

student@lab:~$ ls

linux/
bash/
networking/
web/
privilege-escalation/

student@lab:~$`}
              </pre>
            </div>
          </div>
        </div>
      </section>
    </main>
  );
}
