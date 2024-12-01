#include "hand.h"

// ------------------------------------------------------------------------------------------------- Main ---
int main(int argc, char *argv[]) {

  std::string line;
  std::ifstream file("input.txt");

  std::vector<Hand> hands;

  if (!file.is_open()) {
    std::cout << "Unable to open file";
    return 1;
  } else {

    while (getline(file, line)) {
      size_t space_pos = line.find_first_of(' ');

      std::string value = line.substr(0, space_pos);
      int bid = std::stoi(line.substr(space_pos + 1, -1));
      Hand hand(std::move(value), bid);

      hands.emplace_back(hand);
    }

    file.close();
  }

  // --------------------------------------------------------------------------------------------------------
  std::sort(hands.begin(), hands.end());

  int total = 0;
  for (int i = 0; i < hands.size(); ++i)
    total += hands[i].bid() * (i + 1);

  std::cout << total << "\n";

  return 0;
}
