export default function MarkdownWidget({ task }) {
  if (!task) return null;

  return (
    <div>
      <h2 className="text-2xl font-bold text-white mb-6">{task.title}</h2>

      <div className="text-zinc-300 whitespace-pre-line leading-7">
        {task.content}
      </div>
    </div>
  );
}
