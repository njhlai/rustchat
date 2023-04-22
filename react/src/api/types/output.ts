import { Change, Message, User } from "./data";

export enum OutputTypes {
    Error = "Error",
    Alive = "Alive",
    CurrentState = "CurrentState",
    UserJoined = "UserJoined",
    UserLeft = "UserLeft",
    Posted = "Posted",
}

export enum OutputErrors {
    UserAlreadyJoined = "UserAlreadyJoined",
}

export interface ErrorOutput {
    type: OutputTypes.Error;
    payload: OutputErrors;
}

export interface AliveOutput {
    type: OutputTypes.Alive;
}

export interface CurrentStateOutput {
    type: OutputTypes.CurrentState;
    payload: {
        myself: User;
        users: User[];
        messages: Message[];
    };
}

export interface UserJoinedOutput {
    type: OutputTypes.UserJoined;
    payload: Change;
}

export interface UserLeftOutput {
    type: OutputTypes.UserLeft;
    payload: Change;
}

export interface PostedOutput {
    type: OutputTypes.Posted;
    payload: {
        message: Message;
    };
}

export type Output =
    | ErrorOutput
    | AliveOutput
    | CurrentStateOutput
    | UserJoinedOutput
    | UserLeftOutput
    | PostedOutput;
