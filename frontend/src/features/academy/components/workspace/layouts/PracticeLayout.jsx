import LearningPanel from "../panels/LearningPanel";
import InteractionPanel from "../panels/InteractionPanel";

import MarkdownWidget from "../widgets/MarkdownWidget";
import TerminalWidget from "../widgets/TerminalWidget";

import Card from "../../../../../shared/ui/Card";

export default function PracticeLayout({ task }) {
  return (
    <div className="flex gap-6 w-full h-full min-h-0">
      {/* Learning side */}
      <div className="basis-[45%] h-full">
        <LearningPanel>
          <MarkdownWidget task={task} />
        </LearningPanel>
      </div>

      {/* Interaction side */}
      <div className="basis-[55%] h-full">
        <InteractionPanel>
          <Card className="h-full">
            <TerminalWidget task={task} />
          </Card>
        </InteractionPanel>
      </div>
    </div>
  );
}
