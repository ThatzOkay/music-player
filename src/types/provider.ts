import { ProviderType } from "./providerType";

export interface Provider {
    id:              number;
    name:            string;
    api:             string;
    username:        string;
    password:        string;
    ip:              string;
    port:            number;
    connection_type: ProviderType;
}
