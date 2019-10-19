interface IResponder {
    hello: () => string
}

const responder: IResponder = {
    "hello": () => "hello"
}

console.log(responder.hello())