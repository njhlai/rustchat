import { v4 } from "uuid";

export interface User {
    id: typeof v4;
    name: string;
}

export interface Message {
    id: typeof v4;
    sender: typeof v4;
    timestamp: EpochTimeStamp;
    body: string;
}

export interface Feed {
    users: User[];
    messages: Message[];
}

export interface Change {
    user: User;
    timestamp: EpochTimeStamp;
}
