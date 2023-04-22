import { PayloadAction, createSlice } from "@reduxjs/toolkit";

import { User } from "../types/data";

const userSlice = createSlice({
    name: "user",
    initialState: null as User | null,
    reducers: {
        Joined(state: User | null, action: PayloadAction<User>) {
            return action.payload;
        },
    },
});

export const { Joined } = userSlice.actions;
export default userSlice.reducer;
