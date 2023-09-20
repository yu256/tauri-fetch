import { Prism } from "react-syntax-highlighter";
import { base16AteliersulphurpoolLight } from "react-syntax-highlighter/dist/esm/styles/prism";
import { tryFormatJson } from "./scripts/json.ts";

type Props = {
  json: string;
};

export default function RenderJson({ json }: Props) {
  return (
    <div className="m-5 rounded-lg overflow-clip">
      <Prism language="json" style={base16AteliersulphurpoolLight}>
        {tryFormatJson(json)}
      </Prism>
    </div>
  );
}
