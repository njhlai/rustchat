import { call, fork, put, take } from "@redux-saga/core/effects";
import { EventChannel, eventChannel } from "redux-saga";

import { Input, InputTypes } from "../types/input";
import { Output, OutputTypes } from "../types/output";
import { Joined } from "../user/slice";
import { Load, UserJoined, UserLeft } from "../feed/slice";

function createWebSocketChannel(ws: WebSocket) {
    return eventChannel<Output>((emit) => {
        ws.onmessage = (event) => {
            const output = JSON.parse(event.data as string) as Output;

            emit(output);
        };

        return () => ws.close();
    });
}

function* read(wsChannel: EventChannel<Output>) {
    for (;;) {
        const output = (yield take(wsChannel)) as Output;

        switch (output.type) {
            case OutputTypes.CurrentState:
                console.log("Logged in as:", output.payload.myself);
                console.log("Current state of server:", output.payload);

                yield put(Joined(output.payload.myself));
                yield put(
                    Load({
                        users: output.payload.users,
                        messages: output.payload.messages,
                    })
                );
                break;
            case OutputTypes.Posted:
                console.log(
                    output.payload.message.sender,
                    "@",
                    output.payload.message.timestamp,
                    ":",
                    output.payload.message.body
                );

                break;
            case OutputTypes.UserJoined:
                console.log(output.payload.user.id, "joined the server");

                yield put(UserJoined(output.payload));
                break;
            case OutputTypes.UserLeft:
                console.log(output.payload.user.id, "left the server");

                yield put(UserLeft(output.payload));
                break;
            case OutputTypes.Error:
                console.log("ERR: Code:", output.payload);

                break;
            default:
                console.log("Received other request:", output);

                break;
        }
    }
}

function* write(ws: WebSocket) {
    for (;;) {
        const input = (yield take(Object.values(InputTypes))) as Input;

        ws.send(JSON.stringify(input));
    }
}

function* connectWebSocket() {
    const isBrowser = typeof window != "undefined";
    const ws = isBrowser ? new WebSocket("ws://localhost:8080/feed") : null;

    if (ws) {
        const wsChannel = (yield call(
            createWebSocketChannel,
            ws
        )) as EventChannel<Output>;

        yield fork(read, wsChannel);
        yield fork(write, ws);
    }
}

export default function* wsSaga() {
    yield call(connectWebSocket);
}
