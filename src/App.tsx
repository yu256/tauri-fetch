import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Prism } from "react-syntax-highlighter";
import { base16AteliersulphurpoolLight } from "react-syntax-highlighter/dist/esm/styles/prism";
import { writeText } from "@tauri-apps/api/clipboard";

type Arguments = {
  method: string;
  target: string;
  body?: string;
  headers: Map<string, string>; // todo
};

const args = ["method", "target", "body"] as const;

const argValue: Arguments = {
  method: "",
  target: "",
  headers: new Map(),
};

export default function App() {
  const [res, setRes] = useState("");

  return (
    <div className="text-center mt-10">
      {args.map(arg => (
        <input
          className="m-3"
          onChange={e => (argValue[arg] = e.currentTarget.value)}
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
          <div className="m-5 rounded-lg overflow-clip">
            <Prism language="json" style={base16AteliersulphurpoolLight}>
              {JSON.stringify(JSON.parse(res), null, 2)}
            </Prism>
          </div>
        </>
      )}
    </div>
  );
}
