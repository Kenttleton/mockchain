# Node

This example uses UDP Multicast to truly decentralize communication. A node just needs to subscribe to the network and will receive new blocks as they are broadcast. Limitations exist as a result of choosing UDP. Datagram size limits are 1500 bytes which is several hundred times smaller than the 1 MB max block size Bitcoin uses. As a result, encryption was forgone due to size constraints. A chunking or slicing logic could get around the size limitation but requires a larger stack size for the thread to handle the volume, dealing with dropped or out of order datagrams, etc.
