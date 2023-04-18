import { CSSProperties } from "react";

import Login from "../components/login";
import ParticipantWindow from "../components/participant";

const styles = {
    main: {
        borderTopStyle: "solid",
        display: "flex",
        flexDirection: "row",
        height: "100%",
    },
} as Record<string, CSSProperties>;

export const metadata = {
    title: "rustchat",
    description: "React frontend for rustchat",
};

export default function Page() {
    return (
        <>
            <h1>{metadata.title}@</h1>
            <div style={styles.main}>
                <ParticipantWindow />
                <Login />
            </div>
        </>
    );
}
