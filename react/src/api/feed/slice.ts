import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { Feed, Message, ActivityTimeStamp } from "../types/data";
import { load, message, userJoin, userLeft } from "./actions";

const feedSlice = createSlice({
    name: "feed",
    initialState: {
        users: [],
        activities: [],
    } as Feed,
    reducers: {
        Load(state: Feed, action: PayloadAction<Feed>) {
            action.payload.activities = [...action.payload.activities, load()];

            return action.payload;
        },
        Posted(state: Feed, action: PayloadAction<Message>) {
            return {
                ...state,
                activities: [...state.activities, message(action.payload)],
            };
        },
        UserJoined(state: Feed, action: PayloadAction<ActivityTimeStamp>) {
            return {
                ...state,
                users: [...state.users, action.payload.user],
                activities: [...state.activities, userJoin(action.payload)],
            };
        },
        UserLeft(state: Feed, action: PayloadAction<ActivityTimeStamp>) {
            return {
                ...state,
                users: state.users.filter(
                    (user) => user.id != action.payload.user.id
                ),
                activities: [...state.activities, userLeft(action.payload)],
            };
        },
    },
});

export const { Load, Posted, UserJoined, UserLeft } = feedSlice.actions;
export default feedSlice.reducer;
