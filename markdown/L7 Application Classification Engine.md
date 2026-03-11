---
itemId: L7-Application-Classification-Engine
itemType: Software Item Spec
itemFulfills: CS-1,CS-2,CS-3,KD-20
---

# L7 Application Classification Engine (Markdown)

The NAT engine shall implement: (1) Static NAT via direct mapping table, (2) Dynamic NAT via pool allocator with LRU eviction, (3) PAT via port block allocation (512 ports per block) with randomized port selection, (4) Twice NAT via chained translation entries. Translation table shall use the same Cuckoo hash structure as the connection table. ALG (Application Layer Gateway) modules for FTP, SIP, H.323, and PPTP shall modify embedded addresses in payload.
