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

export interface ActivityTimeStamp {
    user: User;
    timestamp: EpochTimeStamp;
}

export enum ActivityTypes {
    Load = "Load",
    UserJoined = "UserJoined",
    UserLeft = "UserLeft",
    Message = "Message",
}

export interface LoadActivity {
    type: ActivityTypes.Load;
}

export interface MessageActivity {
    type: ActivityTypes.Message;
    event: Message;
}

export interface UserJoinedActivity {
    type: ActivityTypes.UserJoined;
    event: ActivityTimeStamp;
}

export interface UserLeftActivity {
    type: ActivityTypes.UserLeft;
    event: ActivityTimeStamp;
}

export type Activity =
    | LoadActivity
    | MessageActivity
    | UserJoinedActivity
    | UserLeftActivity;

export interface Feed {
    users: User[];
    activities: Activity[];
}
