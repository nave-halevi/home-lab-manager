export default function Dashboard() {
  const user = "Nave";

  const tracks = [
    { name: "Linux Fundamentals", progress: 80 },
    { name: "Bash Scripting", progress: 60 },
    { name: "Networking Basics", progress: 35 },
  ];

  return (
    <main className="min-h-screen bg-zinc-950 text-white px-6 py-10">
      {/* Header */}
      <section className="mb-10">
        <h1 className="text-3xl font-bold">
          Welcome back, <span className="text-red-500">{user}</span>
        </h1>

        <p className="text-zinc-400 mt-2">
          Continue your cyber security learning path.
        </p>
      </section>

      {/* Progress */}
      <section className="mb-10">
        <h2 className="text-xl font-semibold mb-4">Your Progress</h2>

        <div className="grid md:grid-cols-3 gap-6">
          {tracks.map((t, i) => (
            <div
              key={i}
              className="rounded-xl border border-zinc-800 bg-zinc-900 p-5"
            >
              <h3 className="font-semibold">{t.name}</h3>

              <div className="mt-4 h-2 bg-zinc-800 rounded-full">
                <div
                  className="h-2 bg-red-600 rounded-full"
                  style={{ width: `${t.progress}%` }}
                />
              </div>

              <p className="text-sm text-zinc-400 mt-2">
                {t.progress}% completed
              </p>
            </div>
          ))}
        </div>
      </section>

      {/* Continue Learning */}
      <section className="mb-10">
        <h2 className="text-xl font-semibold mb-4">Continue Learning</h2>

        <div className="grid md:grid-cols-2 gap-6">
          {["Linux Fundamentals", "Bash Scripting"].map((item, i) => (
            <div
              key={i}
              className="rounded-xl border border-zinc-800 bg-zinc-900 p-5 hover:border-zinc-700 transition"
            >
              <h3 className="font-semibold">{item}</h3>

              <p className="text-zinc-400 mt-2 text-sm">
                Resume your learning journey in this track.
              </p>

              <button className="mt-4 text-red-500 hover:text-red-400">
                Continue →
              </button>
            </div>
          ))}
        </div>
      </section>

      {/* Bottom Widgets */}
      <section className="grid md:grid-cols-2 gap-6">
        <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-5">
          <h3 className="font-semibold mb-3">Recent Activity</h3>
          <ul className="text-sm text-zinc-400 space-y-2">
            <li>✔ Completed Linux permissions lab</li>
            <li>✔ Solved Bash loops challenge</li>
            <li>✔ Viewed networking module</li>
          </ul>
        </div>

        <div className="rounded-xl border border-zinc-800 bg-zinc-900 p-5">
          <h3 className="font-semibold mb-3">Daily Challenge</h3>

          <p className="text-sm text-zinc-400">
            Exploit a misconfigured SSH service and gain access.
          </p>

          <button className="mt-4 bg-red-600 hover:bg-red-700 px-4 py-2 rounded-lg">
            Start Challenge
          </button>
        </div>
      </section>
    </main>
  );
}
