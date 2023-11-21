import { Button } from "@mui/material";
import { Props } from "../types/props"

function ProviderSelect({ children, onClick }: Props & { onClick: () => void }) {
    return (<Button onClick={onClick} variant="contained">{children}</Button>);
}

export default ProviderSelect