#include <stdio.h>
#include <stdlib.h>
#include <sys/socket.h>
#include <ifaddrs.h>
#include <netdb.h>


int main() {
    struct ifaddrs *addresses;
    struct ifaddrs *address = addresses;

    if (getifaddrs(&addresses) == -1) {
        printf("Failed to initialize.\n");
        return -1;
    }

    while (address) {
        int family = address ->ifa_addr->sa_family;
        if (family == AF_INET || family == AF_INET6) {
            printf("%s\t", address->ifa_name);
            printf("%s\t", family == AF_INET ? "IPV4": "IPV6");
            char ap[100];
            const int family_size = family == AF_INET ?
                sizeof(struct sockaddr_in) : sizeof(struct sockaddr_in6);
            getnameinfo(address->ifa_addr,
                    family_size, ap, sizeof(ap), 0, 0, NI_NUMERICHOST);
            printf("%s\t", ap);
        }
        address = address->ifa_next;

    }
    freeifaddrs(addresses);

    return 0;
}
