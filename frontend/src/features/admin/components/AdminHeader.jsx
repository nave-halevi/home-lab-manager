export default function AdminHeader({ title, description, action }) {
  return <div className="mb-7 flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"><div><h1 className="text-3xl font-bold">{title}</h1><p className="mt-2 text-zinc-400">{description}</p></div>{action}</div>;
}
