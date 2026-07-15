import { useLabs } from "../hooks/useLabs";
import CreateLabButton from "../components/CreateLabButton";
import LabWorkspace from "../components/LabWorkspace";

const LabsPage = () => {
  const { activeLab, isLoading, error, handleCreateLab, handleDeleteLab } = useLabs();

  if (activeLab) {
    return <LabWorkspace activeLab={activeLab} onDeleteLab={handleDeleteLab} />;
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center text-center px-6">
      <h1 className="text-3xl font-bold mb-6">Cyber Security Home Lab 🚀</h1>

      <CreateLabButton isLoading={isLoading} onCreate={handleCreateLab} />

      {error && (
        <div className="mt-6 text-red-400 bg-red-950/30 border border-red-900 px-4 py-2 rounded-lg">
          [ERROR] {error}
        </div>
      )}
    </div>
  );
};

export default LabsPage;
