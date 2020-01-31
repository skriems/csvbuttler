vcl 4.0;

backend default {
  .host = "csvbuttler:8000";
}

sub vcl_recv {
  # save the cookies before the built-in vcl_recv
  set req.http.Cookie-Backup = req.http.Cookie;
  unset req.http.Cookie;
}

sub vcl_hash {
  if (req.http.Cookie-Backup) {
    # restore the cookies before the lookup if any
    set req.http.Cookie = req.http.Cookie-Backup;
    unset req.http.Cookie-Backup;
  }
}
