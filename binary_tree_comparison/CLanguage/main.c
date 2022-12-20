#define NULL ((void*) 0)
typedef int BOOL;
#define TRUE (-1)
#define FALSE (0)
#define ASSERT(c, s) if (!c) { printf("ASSERT FAILED: "); puts(s); exit(-1); }
#if 1
#define DEBUG(s) 
#else
#define DEBUG(s) {s}
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct node {
    char * payload;
    struct node * left;
    struct node * right;
} NODE, *PNODE;

PNODE create_node (char * payload)
{
    PNODE result = calloc(1, sizeof(NODE));
    result->left = NULL;
    result->right = NULL;
    result->payload = strdup(payload);
}

void delete_node(PNODE this)
{
    free(this->payload);
    free(this);
}

void delete_recursively(PNODE root)
{
    free(root->payload);
    if (root->left != NULL) {
        delete_recursively(root->left);
    }
    if (root->right != NULL) {
        delete_recursively(root->right);
    }
    free(root);
}


BOOL try_get_identifier(char ** p, char ** result, char ** error_message)
{
    * result = "";
    * error_message = "";

    if (**p != '"') 
    {
        *error_message = "Incorrect start of identifier";
        return FALSE;
    }

    char * pp;
    for (pp = *p+1; *pp != '"'; pp++) {
        if (*pp == '\0') {
            *error_message ="Identifier not ending";
            return FALSE;
        }
    }

    int identifier_size =  pp-(*p);
    * result = calloc(identifier_size, sizeof(char));
    char *p1 = *p+1;
    for (int i= 0; p1<pp; p1++, i++) {
        (*result)[i] = *p1;
        (*result)[i+1] = '\0';
    }

    *p = pp+1;
    return TRUE;
}

/*
 * Interprets the tree_descriptor string following the recursive form
 *  <node_descriptor> ::= (payload,<left_subtree>,<right_subtree>) 
 *  <left_subtree> ::= 0 | <node_descriptor>
 *  <right_subtree> :: 0 | <node_descriptor>
 * This is a valid string 
 *   ("A", ("B", 0,0), ("C", ("D", 0,0),("E", ("F", 0,0),("G", 0, 0)))) 
 * and represents the binary tree
 * A
 * +->B
 * | +->0
 * | +->0
 * +->C
 *   +->D
 *   |  +->0
 *   |  +->0
 *   *->E
 *      +->F
 *      |  +->0
 *      |  +->0
 *      +->G
 *         +->0
 *         +->0
 */
BOOL try_build_tree_from_string(PNODE * root, char ** error_message, char ** tree_descriptor)
{
    DEBUG(printf("0 %s\n", *tree_descriptor);)
    if (*(*tree_descriptor)++ != '(') {
        *error_message = "Incorrect start of subtree";
        *root = NULL;
        return FALSE;
    }

    char *identifier = NULL;

    DEBUG(printf("1 %s\n", *tree_descriptor);)
    if (! try_get_identifier(tree_descriptor, &identifier, error_message)) {
        *root = NULL;
        return FALSE;
    }

    DEBUG(printf("2 %s\n", *tree_descriptor);)
    *root = create_node(identifier);
    if (*(*tree_descriptor)++ != ',') {
        *error_message = "Incorrect start of left subtree";
        *root = NULL;
        return FALSE;
    }

    DEBUG(printf("3 %s\n", *tree_descriptor);)
    if (**tree_descriptor == '(') {
        if (! try_build_tree_from_string(&((*root)->left), error_message, tree_descriptor)) {
            return FALSE;
        }
    } else if (**tree_descriptor == '0') {
        * (*tree_descriptor)++;
        (*root)-> left = NULL;
    } else {
        *error_message = "Incorrect left  subtree";
        return FALSE;
    }

    DEBUG(printf("4 %s\n", *tree_descriptor);)
    if (*(*tree_descriptor)++ != ',') {
        *error_message = "Incorrect start of right subtree";
        *root = NULL;
        return FALSE;
    }

    DEBUG(printf("5 %s\n", *tree_descriptor);)
    if (**tree_descriptor == '(') {
        if (! try_build_tree_from_string(&((*root)->right), error_message, tree_descriptor)) {
            return FALSE;
        }
    } else if (**tree_descriptor == '0') {
        * (*tree_descriptor)++;
        (*root)-> left = NULL;
    } else {
        *error_message = "Incorrect right subtree";
        return FALSE;
    }

    if (*(*tree_descriptor)++ != ')') {
        *error_message = "Incorrect end of subtree";
        *root = NULL;
        return FALSE;
    }
}

void print_tree(int left_indent, PNODE root)
{
    for (int i=0; i<left_indent; i++) {
        putc(' ', stdout);
    }
    if (root == NULL) {
        puts("0");
        return;
    }

    puts(root->payload);
    print_tree(left_indent + 2, root->left);
    print_tree(left_indent + 2, root->right);
}

// void visit_preorder(PNODE root, void (*iterator)())

/*** UNIT TESTS */
void try_get_identifier_test_case(char *case_name, char *expected, char * test, char * error_if_fails)
{
    char * identifier;
    char * error;
    BOOL esito = try_get_identifier(&test, &identifier, &error);
    if (!esito) {
        if (strcmp(error, error_if_fails) != 0) {
            ASSERT(esito, error);
        }
    }

    ASSERT(strcmp(expected, identifier)==0, "Differing strings");
    printf("Test case %s: OK\n", case_name);
}

void try_get_identifier_test_case_returns_the_remaining_string(char *case_name, char *expected_remaining, char * test)
{
    char * identifier;
    char * error;
    char ** ptest = &test;
    BOOL esito = try_get_identifier(ptest, &identifier, &error);
    ASSERT(esito,"Error");
    ASSERT(strcmp(expected_remaining, *ptest) == 0, "Remaining differs");
    printf("Test case %s: OK\n", case_name);
}

//void try_build_tree_from_string_test_case(char *case_name)

int main()
{
    PNODE root;

#if 0
    // try_get_identifier test case
    try_get_identifier_test_case("1. case", "1234567", "\"1234567\"", "");
    try_get_identifier_test_case("2. case", "", "\"\"", "");
    try_get_identifier_test_case("3. error case", "", "\"", "Identifier not ending");
    try_get_identifier_test_case("4. error case", "", "", "Incorrect start of identifier");
    try_get_identifier_test_case_returns_the_remaining_string("5. case", "Remaining", "\"1234567\"Remaining");
#endif

#if 1
    root = create_node("1");
    print_tree(0, root);
    puts("-----------");
    root->left = create_node("1.1");
    root->left->left = create_node("1.1.1");

    root->right = create_node("1.2");
    root->right->left = create_node("1.2.1");
    print_tree(0, root);
    delete_recursively(root);
#endif


#if 0
    char * error_message;
    char * prefix = "(\"A\",0,0)";
    if (try_build_tree_from_string(&root, &error_message, &prefix)) {
        print_tree(0, root);
    } else {
        printf("ERRORE : %s\n", error_message);
    }
    delete_recursively(root);
    prefix = "(\"A\",(\"B\",0,0),(\"C\",(\"D\",0,0),(\"E\",(\"F\",0,0),(\"G\",0,0)))))";
    if (try_build_tree_from_string(&root, &error_message, &prefix)) {
        print_tree(0, root);
    } else {
        printf("ERRORE : %s\n", error_message);
    }
#endif
}