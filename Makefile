CXX		?=	g++

RM		?=	rm -f

SRC		=	ll_lexer.cpp

SRC		:=	$(addprefix src/, $(SRC))

OBJ		=	$(SRC:.cpp=.o)

CXXFLAGS+=	-Wall -Wextra

NAME	=	test_bin

all:		$(NAME)

$(NAME):	$(OBJ)
			$(CXX) -o $(NAME) $(OBJ)

clean:
			$(RM) $(OBJ)

fclean:		clean
			$(RM) $(NAME)

re:			fclean	all

.PHONY:		all clean fclean re
