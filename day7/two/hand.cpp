#include "hand.h"

Hand::Hand(std::string &&value, int bid) : _value(value), _bid(bid), _type(Type::HIGH_CARD) {

  // Get the type --
  // ! Do not sort _value (elf rule)
  std::string sorted_value(_value);
  std::sort(sorted_value.begin(), sorted_value.end());

  int16_t duplicates[3] = {0, 0, 0}; // Nb J in third position
  int16_t index = 0;

  for (int16_t i = 0; i < sorted_value.size(); ++i) {

    if (sorted_value[i] == 'J')
      duplicates[2]++;
    else if (sorted_value[i] == sorted_value[i + 1])
      duplicates[index]++;
    else if (duplicates[index] != 0)
      index++;
  }

  // Add J to duplicates
  if (duplicates[0] >= duplicates[1])
    duplicates[0] += duplicates[2];
  else if (duplicates[0] < duplicates[1])
    duplicates[1] += duplicates[2];

  // --
  if (duplicates[0] >= 4)
    _type = Type::FIVE_KIND;
  else if (duplicates[0] == 3 || duplicates[1] == 3)
    _type = Type::FOUR_KIND;
  else if ((duplicates[0] == 2 && duplicates[1] == 1) || (duplicates[0] == 1 && duplicates[1] == 2))
    _type = Type::FULL_HOUSE;
  else if (duplicates[0] == 2 || duplicates[1] == 2)
    _type = Type::THREE_KIND;
  else if (duplicates[0] == 1 && duplicates[1] == 1)
    _type = Type::TWO_PAIR;
  else if (duplicates[0] == 1 || duplicates[1] == 1)
    _type = Type::ONE_PAIR;
  else
    _type = Type::HIGH_CARD;
}

// ------------------------------------------------------------------------------------------ Operators ---
bool operator==(const Hand &a, const Hand &b) { return a._type == b._type; }
bool operator<(const Hand &a, const Hand &b) {

  // Elf rule: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J

  if (a == b) {
    for (int16_t i = 0; i < a._value.size(); ++i) {
      if (a._value[i] != b._value[i]) {

        if (a._value[i] == 'J')
          return true;
        if (b._value[i] == 'J')
          return false;
        if (std::isdigit(a._value[i]) && std::isdigit(b._value[i]))
          return a._value[i] < b._value[i];
        if (std::isdigit(a._value[i]))
          return true;
        if (std::isdigit(b._value[i]))
          return false;
        if (a._value[i] == 'T')
          return true;
        if (a._value[i] == 'Q' && (b._value[i] == 'K' || b._value[i] == 'A'))
          return true;
        if (a._value[i] == 'K' && b._value[i] == 'A')
          return true;
        else
          return false;
      }
    }
  }

  return a._type < b._type;
}

// --------------------------------------------------------------------------------------------------------
int Hand::bid() const { return _bid; }
