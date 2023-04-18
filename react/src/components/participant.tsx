import { CSSProperties } from "react";

const styles = {
    participants: {
        borderRightStyle: "solid",
        flexGrow: 1,
        marginTop: 0,
    },
} as Record<string, CSSProperties>;

export default function ParticipantWindow() {
    return (
        <>
            <div style={styles.participants}>
                <h3>Participants</h3>
            </div>
        </>
    );
}
