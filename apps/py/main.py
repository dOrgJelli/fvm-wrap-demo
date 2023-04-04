import asyncio

from polywrap_client import ( PolywrapClient )
from polywrap_core import ( Uri, InvokerOptions )

async def encode_message():
    # Instantiate the client
    client = PolywrapClient()
    
    # Load the WebAssembly wrapper through a URI that points to local file system
    uri = Uri('wrap://file/../../build/')

    args = {
        "message": {
          "actor": "foo-bar",
          "args": '{ "arg1": [1, 2, 3] }'
        }
    }

    # Configure the client
    options = InvokerOptions(
        uri=uri,
        method="encodeMessage",
        args=args,
        encode_result=False
    )
    
    # Invoke the wrapper 
    result = await client.invoke(options)

    if result.ok:
        print([x for x in result.value])
    else:
        print(result.error)

if __name__ == "__main__":
    asyncio.run(encode_message())
