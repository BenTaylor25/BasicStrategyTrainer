# Basic Strategy Trainer

Interactive endless quiz for learning Blackjack [Basic Strategy](https://en.wikipedia.org/wiki/Blackjack#Basic_strategy).  

Casino Blackjack is a card game where 1+ players play against a dealer.  
Players start with two face-up cards and the dealer starts with one face-up and one face-down card.  
The players' goal is to draw cards from the shoe(1-8 decks of cards) to reach a higher value than the dealer,  
however if the total exceeds 21, that's a bust (automatic loss).  
Numbered cards have their number's value; Jacks, Queens, and Kings are worth 10; and Aces can be worth 1 or 11 e.g. A+5 = 16 (*because 16 is better than 6*), A+6+7 = 14 (*14 is better as 24 is bust*).

After the cards are dealt, the player moves first, and has several choices:
- Hit (add a new card to their total), 
- Stand (keep current total, and pass turn to next player), 
- Double Down (if the player hasn't hit, double the bet, add 1 new card to the total, and pass turn on), 
- Split (if the player hasn't hit, and if both starting cards are the same, split cards into two hands, add a bet to the new hand),
- Surrender (keep half of your bet, sit out the round).

For every move the player can make, there is an optimal decision: Basic Strategy.  
Basic Strategy will help you win more games.

**Note: Basic Strategy does NOT give you enough of an advantage to win more than you lose.**  
**Note: I do not advocate careless gambling; please do not gamble what you cannot afford to lose.**

![image](https://user-images.githubusercontent.com/97246704/208298259-b315bf41-57dc-4a55-a087-3830bae04c57.png)


(todo)
- implement surrender
- refactor split (das)?

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

Double after Split (DAS) is a rule variation that some casinos will-, and some casinos will not have.  
If you have two of the same card and Split, can you Double Down on one of your two hands?

---

## Surrender
Player 16 against a 9, T, or A should surrender.  
Player 15 against a T should surrennder.
