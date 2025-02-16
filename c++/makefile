CXX = clang++
CXXFLAGS = -Wall -std=c++17
MODE_COMPILATION=-O2

SRC_DIR=src
DATA_DIR=../instances
OUTPUT_DIR=output

EXE_PROBLEM=$(OUTPUT_DIR)/leer_ejemplar
INST_TEST=$(DATA_DIR)/abz7.txt

prueba_lectura: $(INST_TEST) ${EXE_PROBLEM}
	./${EXE_PROBLEM} $(INST_TEST)

${OUTPUT_DIR}:
	mkdir -p ${OUTPUT_DIR}

${EXE_PROBLEM}: src/problem/*.cpp 
	$(CXX) $(MODE_COMPILATION) -o $@ $^ $(CXXFLAGS)

EXE_RANDOM_SOL=output/random_sol 

prueba_random_sol: $(INST_TEST) $(EXE_RANDOM_SOL)
	./${EXE_RANDOM_SOL} $(INST_TEST)

${EXE_RANDOM_SOL}: src/problem/instance.cpp src/solution/*.cpp
	$(CXX) $(MODE_COMPILATION) -o $@ $^ $(CXXFLAGS)
