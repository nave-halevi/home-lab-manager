import Card from "../../../../../shared/ui/Card";

export default function LearningPanel({ children, className = "" }) {
  return (
    <div
      className={`
        flex-1
        min-h-0
        min-w-0
        ${className}
      `}
    >
      <Card className="h-full min-h-0 overflow-y-auto">{children}</Card>
    </div>
  );
}
