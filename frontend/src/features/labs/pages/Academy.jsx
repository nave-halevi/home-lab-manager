import { useNavigate } from "react-router-dom";

export default function Academy() {
  const navigate = useNavigate();

  const courses = [
    {
      title: "Linux Fundamentals",
      desc: "File system, permissions, processes",
    },
    {
      title: "Bash Scripting",
      desc: "Automation, loops, scripts, variables",
    },
    {
      title: "Networking Basics",
      desc: "TCP/IP, DNS, HTTP, ports",
    },
  ];

  return (
    <main className="min-h-screen bg-zinc-950 text-white px-6 py-10">
      {/* Header */}
      <section className="mb-10">
        <h1 className="text-3xl font-bold">Academy</h1>
        <p className="text-zinc-400 mt-2">
          Choose a learning path and start practicing.
        </p>
      </section>

      {/* Courses */}
      <section className="grid md:grid-cols-3 gap-6">
        {courses.map((c, i) => (
          <div
            key={i}
            className="rounded-xl border border-zinc-800 bg-zinc-900 p-5 hover:border-zinc-700 transition"
          >
            <h2 className="text-lg font-semibold">{c.title}</h2>

            <p className="text-sm text-zinc-400 mt-2">{c.desc}</p>

            <button
              onClick={() => navigate("/labs")}
              className="mt-4 text-red-500 hover:text-red-400"
            >
              Start Course →
            </button>
          </div>
        ))}
      </section>
    </main>
  );
}
