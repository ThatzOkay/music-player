import { Button } from "@chakra-ui/react";
import { Props } from "../types/props"

function ProviderSelect({ children, onClick }: Props & { onClick: () => void }) {
    return (<Button onClick={onClick} colorScheme="blue" >{children}</Button>);
}

export default ProviderSelect