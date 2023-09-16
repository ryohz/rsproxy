import { capitalize } from "./common";

export class Request {
    headers: string;
    version: string;
    method: string;
    url: string;
    body: string;
    piloted: boolean;
    pair_id: string;

    constructor(args: RustRequest) {
        this.headers = args.headers;
        this.version = args.version;
        this.method = args.method;
        this.url = args.url;
        this.body = args.body;
        this.piloted = args.piloted;
        this.to_editable();

        let headers: Record<string, string> = JSON.parse(this.headers);
        this.pair_id = headers["Pair-Id"];
    }

    to_editable(): string {
        let rq = "";
        let headers: Record<string, string> = JSON.parse(this.headers);

        // method path version
        let host = headers['host'];
        let path = "/"
        if (host !== undefined) {
            path = this.url.split(host)[1];
        }
        rq += `${this.method.toUpperCase()} ${path} ${this.version}\n`;

        // headers
        rq += headers_to_editable(headers);

        // body
        rq += "\n"
        rq += this.body;

        return rq;
    }
}

export class Response {
    headers: string;
    version: string;
    status: string;
    body: string;
    piloted: boolean;
    pair_id: string;

    constructor(args: RustResponse) {
        this.headers = args.headers;
        this.version = args.version;
        this.status = args.status.toString();
        this.body = args.body;
        this.piloted = args.piloted;

        let headers: Record<string, string> = JSON.parse(this.headers);
        this.pair_id = headers["Pair-Id"];
    }

    to_editable(): string {
        let rs = "";
        let headers: Record<string, string> = JSON.parse(this.headers);

        // version, status
        rs += `${this.version} ${this.status}\n`;

        // headers
        rs += headers_to_editable(headers);

        //body
        rs += "\n"
        rs += this.body;

        return rs;
    }
}

function headers_to_editable(headers: Record<string, string>): string {
    let rq = "";
    for (const key in headers) {
        let parts = key.split("-");
        let part1 = parts[0];
        let header_name = ""
        if (parts.length === 2) {
            let part2 = parts[1];
            part1 = capitalize(part1);
            part2 = capitalize(part2);
            header_name = `${part1}-${part2}`;
        } else if (parts.length === 1) {
            header_name = capitalize(part1);
        }

        if (header_name !== "Pair-Id") {
            rq += `${header_name}: ${headers[key]}\n`;
        }
    }
    return rq;
}

export interface RustRequest {
    headers: string;
    version: string;
    method: string;
    url: string;
    body: string;
    piloted: boolean;
}

export interface RustResponse {
    headers: string;
    version: string;
    status: number;
    body: string;
    piloted: boolean;
}