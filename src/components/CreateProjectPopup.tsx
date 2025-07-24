import { invoke } from "@tauri-apps/api/core";
import { useAtom } from "jotai";
import React from "react";
import { IoClose } from "react-icons/io5";
import { CommandResult } from "../models/commandResult";
import { Project } from "../models/project";
import { allProjectsAtom } from "../stores/projectStore";

interface CreateProjectPopupProp {
  showBanner: (success: boolean, message: string) => void;
  onCloseClick: () => void;
}

const CreateProjectPopup = ({
  showBanner,
  onCloseClick,
}: CreateProjectPopupProp): JSX.Element => {
  const [projects, setProjects] = useAtom(allProjectsAtom);

  const [name, setName] = React.useState<string>("");
  const [description, setDescription] = React.useState<string>("");

  const createProject = async (): Promise<void> => {
    console.log("Creating project");
    const result = await invoke<CommandResult<Project>>(
      "create_project_command",
      { name, description }
    );
    if (!result.success) {
      showBanner(false, result.message ?? "No error message");
      return;
    }
    const projectInfo: Project = {
      id: result.data?.id ?? [],
      name: result.data?.name ?? "",
      description: result.data?.description ?? "",
      boards: [],
    };
    setProjects([...projects, projectInfo]);
    showBanner(true, `Successfully created project: ${projectInfo.name}`);
    onCloseClick();
  };

  return (
    <div className="absolute top-0 left-0 w-screen h-screen bg-black/50 flex">
      <div className="w-3/5 h-3/5 bg-white m-auto relative p-4 rounded-xl">
        <button className="absolute right-4 top-4" onClick={onCloseClick}>
          <IoClose className="cursor-pointer hover:text-black/50" size={30} />
        </button>
        <h1 className="text-2xl font-bold">Create Project</h1>
        <div className="mt-4">
          <p className="text-lg">Project Name:</p>
          <input
            type="text"
            className="bg-[#EFEFEF] w-full text-lg px-2 py-0.5 rounded-lg mt-1"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div className="mt-4">
          <p className="text-lg">Project Description:</p>
          <textarea
            className="bg-[#EFEFEF] w-full h-40 text-lg px-2 py-0.5 rounded-lg mt-1 resize-none"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
        </div>
        <div className="flex w-min ml-auto mt-10">
          <button
            className="bg-blue-600 text-white w-40 mr-3 py-1 text-xl rounded-xl hover:bg-blue-500"
            onClick={onCloseClick}
          >
            Cancel
          </button>
          <button
            className="bg-blue-600 text-white w-40 py-1 text-xl rounded-xl hover:bg-blue-500"
            onClick={createProject}
          >
            Create
          </button>
        </div>
      </div>
    </div>
  );
};

export default CreateProjectPopup;
