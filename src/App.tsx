import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";
import RenderJson from "./RenderJson.tsx";

const args = ["method", "target", "body"] as const satisfies readonly string[];

export default function () {
  const [res, setRes] = useState("");
  const [argValue, setArgValue] = useState({
    method: "",
    target: "",
    body: "",
  });

  useEffect(() => {
    invoke<typeof argValue>("restore").then(setArgValue);
  }, []);

  return (
    <div className="text-center mt-10">
      {args.map(arg => (
        <input
          className="m-3"
          defaultValue={argValue[arg]}
          key={arg}
          onChange={e => {
            argValue[arg] = e.currentTarget.value;
          }}
          placeholder={arg}
        />
      ))}
      <button
        className="m-3"
        onClick={() =>
          invoke<string>("fetch", {
            args: argValue,
          }).then(setRes)
        }
      >
        決定
      </button>

      {res && (
        <>
          <button onClick={() => writeText(res)}>クリップボードにコピー</button>
          <RenderJson json={res} />
        </>
      )}
    </div>
  );
}
