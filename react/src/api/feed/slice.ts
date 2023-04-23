import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { Change, Feed, Message } from "../types/data";

const feedSlice = createSlice({
    name: "feed",
    initialState: {
        users: [],
        prevMessages: [],
        currMessages: [],
    } as Feed,
    reducers: {
        Load(state: Feed, action: PayloadAction<Feed>) {
            return action.payload;
        },
        Posted(state: Feed, action: PayloadAction<Message>) {
            return {
                ...state,
                currMessages: [...state.currMessages, action.payload],
            };
        },
        UserJoined(state: Feed, action: PayloadAction<Change>) {
            return {
                ...state,
                users: [...state.users, action.payload.user],
            };
        },
        UserLeft(state: Feed, action: PayloadAction<Change>) {
            return {
                ...state,
                users: state.users.filter(
                    (user) => user.id != action.payload.user.id
                ),
            };
        },
    },
});

export const { Load, Posted, UserJoined, UserLeft } = feedSlice.actions;
export default feedSlice.reducer;
