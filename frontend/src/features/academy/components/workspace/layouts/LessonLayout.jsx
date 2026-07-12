import LearningPanel from "../panels/LearningPanel";

import MarkdownWidget from "../widgets/MarkdownWidget";

export default function LessonLayout({ task }) {
  return (
    <div className="w-full h-full">
      <LearningPanel>
        <MarkdownWidget task={task} />
      </LearningPanel>
    </div>
  );
}
