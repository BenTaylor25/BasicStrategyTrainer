# Basic Strategy Trainer

Interactive endless quiz for learning Blackjack Basic Strategy.  

---

## Hard Totals
(top = dealer upcard, left = hard totals)  
|   | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | T | A |
| - | - | - | - | - | - | - | - | - | - | - |
|17 | S | S | S | S | S | S | S | S | S | S |
|16 | S | S | S | S | S | H | H | H | H | H |
|15 | S | S | S | S | S | H | H | H | H | H |
|14 | S | S | S | S | S | H | H | H | H | H |
|13 | S | S | S | S | S | H | H | H | H | H |
|12 | H | H | S | S | S | H | H | H | H | H |
|11 | Dh| Dh| Dh| Dh| Dh| Dh| Dh| Dh| Dh| Dh|
|10 | Dh| Dh| Dh| Dh| Dh| Dh| Dh| Dh| H | H |
| 9 | H | Dh| Dh| Dh| Dh| H | H | H | H | H |
| 8 | H | H | H | H | H | H | H | H | H | H |

[S = Stand, H = Hit, Dh = Double Down or Hit]

---

## Soft Totals
(top = dealer updcard, left = soft totals)  
|   | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | T | A |
| - | - | - | - | - | - | - | - | - | - | - |
|A+9| S | S | S | S | S | S | S | S | S | S |
|A+8| S | S | S | S | Ds| S | S | S | S | S |
|A+7| Ds| Ds| Ds| Ds| Ds| S | S | H | H | H |
|A+6| H | Dh| Dh| Dh| Dh| H | H | H | H | H |
|A+5| H | H | Dh| Dh| Dh| H | H | H | H | H |
|A+4| H | H | Dh| Dh| Dh| H | H | H | H | H |
|A+3| H | H | H | Dh| Dh| H | H | H | H | H |
|A+2| H | H | H | Dh| Dh| H | H | H | H | H |

[S = Stand, H = Hit, Dh = Double Down or Hit, Ds = Double Down or Stand]

---

## Split
(top = dealer upcard, left = pair splitting)
|   | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | T | A |
| - | - | - | - | - | - | - | - | - | - | - |
|A+A| Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
|T+T| N | N | N | N | N | N | N | N | N | N |
|9+9| Y | Y | Y | Y | Y | N | Y | Y | N | N |
|8+8| Y | Y | Y | Y | Y | Y | Y | Y | Y | Y |
|7+7| Y | Y | Y | Y | Y | Y | N | N | N | N |
|6+6|Y/N| Y | Y | Y | Y | N | N | N | N | N |
|5+5| N | N | N | N | N | N | N | N | N | N |
|4+4| N | N | N |Y/N|Y/N| N | N | N | N | N |
|3+3|Y/N|Y/N| Y | Y | Y | Y | N | N | N | N |
|2+2|Y/N|Y/N| Y | Y | Y | Y | N | N | N | N |

[Y = Split, N = Don't, Y/N = Split if and only if DAS is allowed]
