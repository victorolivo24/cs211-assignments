#include <stdio.h>
#include <stdlib.h>
#include <string.h>
struct Node {
	int val;
	struct Node* next;
};

struct Node* construct(int val){
	struct Node* N = (struct Node*)malloc(sizeof(struct Node));
	if(N == NULL){
		exit(1);
	}
	N->val = val;
	N->next = NULL;
	return N;
}
void freeList(struct Node* head){
	struct Node* temp;

	while(head!=NULL){
		temp = head;
		head = head->next;
		free(temp);
	}
}
void insert(int val, struct Node** head);
void delete(int val, struct Node** head);
void print(struct Node**head);
int main(int argc, char *argv[]) 
{
	if(argc < 2) {
		fprintf(stderr," ./third {text file}");
		exit(1);
	}
	FILE* file = fopen(argv[1],"r");
	if(file == NULL){
		fprintf(stderr,"%s failed to open",argv[1]);
		exit(1);
	}
	int toInsert;
	char key[7];
	struct Node* head = NULL;
	while(fscanf(file, "%s %d", key, &toInsert) == 2){
		if (strcmp(key, "INSERT") == 0){
			insert(toInsert, &head);
		}
		else if(strcmp(key, "DELETE") == 0){
			delete(toInsert, &head);
		}
		print(&head);
	}
	freeList(head);
	head = NULL;
	fclose(file);
	return EXIT_SUCCESS;
}

void insert(int val, struct Node** head){
	struct Node* newNode = construct(val);
	if(!newNode){
		exit(1);
	}
	if(*head == NULL || (*head)->val >= val){
		newNode->next = *head;
		*head = newNode;
		return;
	} else {
		struct Node* ptr = *head;
		while (ptr != NULL && ptr->next != NULL && ptr->next->val < val) {
				
			ptr = ptr->next;
		}
		newNode->next = ptr->next;
		ptr->next = newNode;
	}

}
void delete(int val, struct Node** head){
	struct Node* prev = NULL;
	struct Node* ptr = *head;
	while(ptr !=NULL) {
		if( ptr->val == val){
			if (prev != NULL){
				prev->next = ptr->next;
			} else {
				*head = ptr->next;
			}

			struct Node* temp = ptr;
			ptr = ptr->next;
			free(temp);
		} else {
		prev = ptr;
		ptr = ptr->next;
		}
	}
}
void print(struct Node** head){
	if(*head == NULL){
		printf("EMPTY\n");
		return;
	}
	struct Node* ptr = *head;
	while(ptr !=NULL){
		printf("%d\t",(*ptr).val);
		ptr = (*ptr).next;
	}
	printf("\n");
}
