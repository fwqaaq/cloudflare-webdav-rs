pub const METHODS: [&str; 12] = [
    "GET",
    "DELETE",
    "PROPPATCH",
    "HEAD",
    "OPTIONS",
    "MKCOL",
    "PROPFIND",
    "COPY",
    "MOVE",
    "LOCK",
    "UNLOCK",
    "PUT",
];

pub const ALLOW_HEADERS: [&str; 6] = [
    "Authorization",
    "Content-Type",
    "Depth",
    "Overwrite",
    "Destination",
    "Range",
];

pub const EXPOSED_HEADERS: [&str; 10] = [
    "Content-Length",
    "Content-Type",
    "Content-Range",
    "Dav",
    "Date",
    "ETag",
    "Last-Modified",
    "Location",
    "Lock-Token",
    "X-WebDAV-Status",
];
