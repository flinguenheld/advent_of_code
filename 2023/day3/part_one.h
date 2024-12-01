#ifndef PART_ONE_H
#define PART_ONE_H

#include <fstream>
#include <iostream>
#include <vector>

class Numbers {

public:
  Numbers();

  void add(char c);
  void set_valid();
  bool valid() const;

  std::vector<int> &values();

private:
  std::vector<int> _values;
  std::string _current;
  bool _valid;
};

#endif // !PART_ONE_H
