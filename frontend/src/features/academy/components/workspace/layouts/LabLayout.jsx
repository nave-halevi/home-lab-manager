import LearningPanel from "../panels/LearningPanel";
import InteractionPanel from "../panels/InteractionPanel";

export default function LabLayout({ task }) {
  return (
    <>
      <LearningPanel>
        <div className="text-white">Lab content coming soon...</div>
      </LearningPanel>

      <InteractionPanel>
        <div className="text-zinc-400">Machine / Terminal / Flag</div>
      </InteractionPanel>
    </>
  );
}
