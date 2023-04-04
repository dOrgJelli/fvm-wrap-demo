import { PolywrapClient } from "@polywrap/client-js";

const client = new PolywrapClient();

async function main() {
  const res = await client.invoke({
    uri: `file/${__dirname}/../../../build/`,
    method: "encodeMessage",
    args: {
      message: {
        actor: "foo-bar",
        args: `{ "arg1": [1, 2, 3] }`
      }
    }
  })

  if (!res.ok)
    console.error(res.error);
  else
    console.log(res.value);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
