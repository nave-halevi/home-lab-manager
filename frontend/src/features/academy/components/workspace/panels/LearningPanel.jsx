import Card from "../../../../../shared/ui/Card";

export default function LearningPanel({ children, className = "" }) {
  return (
    <div
      className={`
        flex-1
        min-w-0
        ${className}
      `}
    >
      <Card className="h-full">{children}</Card>
    </div>
  );
}
