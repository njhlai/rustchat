import { configureStore } from "@reduxjs/toolkit";
import createSagaMiddleware from "redux-saga";

import wsSaga from "../api/websocket/saga";
import feedReducer from "../api/feed/slice";
import useReducer from "../api/user/slice";

const sagaMiddleware = createSagaMiddleware();

const store = configureStore({
    reducer: {
        user: useReducer,
        feed: feedReducer,
    },
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware().concat(sagaMiddleware),
});

sagaMiddleware.run(wsSaga);

export type AppDispatch = typeof store.dispatch;
export type RootState = ReturnType<typeof store.getState>;
export default store;
