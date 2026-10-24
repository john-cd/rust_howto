// ANCHOR: C_DEFINES
#ifndef APP_NAME
#error "APP_NAME must be defined by build.rs"
#endif

#ifndef VERSION
#error "VERSION must be defined by build.rs"
#endif

const char *app_name(void) { return APP_NAME; }

const char *app_version(void) { return VERSION; }

int welcome_is_defined(void) {
#ifdef WELCOME
    return 1;
#else
    return 0;
#endif
}
// ANCHOR_END: C_DEFINES
