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
    </Page>
  );
}

export function Page({ children }) {
  return <main className="p-6 text-white lg:p-10">{children}</main>;
}

function State({ text, error }) {
  return <div className={`p-10 ${error ? "text-red-400" : "text-zinc-400"}`}>{text}</div>;
}
