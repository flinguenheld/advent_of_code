#ifndef HAND_H
#define HAND_H

#include <fstream>
#include <functional>
#include <iostream>

class Hand {

  enum class Type { HIGH_CARD, ONE_PAIR, TWO_PAIR, THREE_KIND, FULL_HOUSE, FOUR_KIND, FIVE_KIND };

public:
  Hand(std::string &&value, int bid);

  int bid() const;

  friend bool operator==(const Hand &a, const Hand &b);
  friend bool operator<(const Hand &a, const Hand &b);

private:
  std::string _value;
  int _bid;
  Type _type;
};

#endif // !HAND_H
