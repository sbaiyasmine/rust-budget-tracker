use std::collections::HashMap;
use dialoguer::{Input, Select};
struct Transaction {
    description: String,
    amount: f64,
}

struct Budget {
    name: String,
    transactions: Vec<Transaction>,
}

impl Budget {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            transactions: Vec::new(),
        }
    }

    fn ajouter_transaction(&mut self, description: String, amount: f64) {
        self.transactions.push(Transaction { description, amount });
        println!("Transaction ajoutée avec succès !");
    }

    fn supprimer_transaction(&mut self, index: usize) {
        if index < self.transactions.len() {
            self.transactions.remove(index);
            println!("Transaction supprimée avec succès !");
        } else {
            println!("Index de transaction invalide !");
        }
    }

    fn modifier_transaction(&mut self, index: usize, description: String, amount: f64) {
        if let Some(transaction) = self.transactions.get_mut(index) {
            transaction.description = description;
            transaction.amount = amount;
            println!("Transaction modifiée avec succès !");
        } else {
            println!("Index de transaction invalide !");
        }
    }

    fn afficher_transactions(&self) {
        println!("\nTransactions pour le budget : {}", self.name);
        for (i, transaction) in self.transactions.iter().enumerate() {
            println!("{}. {} - Montant : {:.2} €", i + 1, transaction.description, transaction.amount);
        }
    }

    fn obtenir_solde(&self) -> f64 {
        self.transactions.iter().map(|t| t.amount).sum()
    }
}

fn main() {
    let mut budgets: HashMap<String, Budget> = HashMap::new();

    loop {
        println!("\nGestionnaire de Budgets");
        let options = vec![
            "Ajouter un budget",
            "Supprimer un budget",
            "Modifier le nom d’un budget",
            "Ajouter une transaction",
            "Modifier une transaction",
            "Supprimer une transaction",
            "Afficher les transactions",
            "Afficher le solde du budget",
            "Quitter",
        ];

        let choix = Select::new()
            .with_prompt("Choisissez une option")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choix {
            0 => {
                let name: String = Input::new()
                    .with_prompt("Nom du nouveau budget")
                    .interact_text()
                    .unwrap();

                if budgets.contains_key(&name) {
                    println!("Un budget avec ce nom existe déjà !");
                } else {
                    budgets.insert(name.clone(), Budget::new(&name));
                    println!("Budget '{}' ajouté avec succès !", name);
                }
            }
            1 => {
                let name: String = Input::new()
                    .with_prompt("Nom du budget à supprimer")
                    .interact_text()
                    .unwrap();

                if budgets.remove(&name).is_some() {
                    println!("Budget '{}' supprimé avec succès !", name);
                } else {
                    println!("Budget '{}' introuvable !", name);
                }
            }
            2 => {
                let old_name: String = Input::new()
                    .with_prompt("Nom du budget à modifier")
                    .interact_text()
                    .unwrap();

                if let Some(budget) = budgets.remove(&old_name) {
                    let new_name: String = Input::new()
                        .with_prompt("Nouveau nom du budget")
                        .interact_text()
                        .unwrap();
                    budgets.insert(new_name.clone(), Budget { name: new_name, ..budget });
                    println!("Nom du budget modifié avec succès !");
                } else {
                    println!("Budget '{}' introuvable !", old_name);
                }
            }
            3 => {
                let name: String = Input::new()
                    .with_prompt("Nom du budget")
                    .interact_text()
                    .unwrap();

                if let Some(budget) = budgets.get_mut(&name) {
                    let description: String = Input::new()
                        .with_prompt("Description de la transaction")
                        .interact_text()
                        .unwrap();
                    let amount: f64 = Input::new()
                        .with_prompt("Montant de la transaction")
                        .interact_text()
                        .unwrap();
                    budget.ajouter_transaction(description, amount);
                } else {
                    println!("Budget '{}' introuvable !", name);
                }
            }
            4 => {
                let name: String = Input::new()
                    .with_prompt("Nom du budget")
                    .interact_text()
                    .unwrap();

                if let Some(budget) = budgets.get_mut(&name) {
                    budget.afficher_transactions();
                    let index: usize = Input::new()
                        .with_prompt("Index de la transaction à modifier")
                        .interact_text()
                        .unwrap();
                    let description: String = Input::new()
                        .with_prompt("Nouvelle description de la transaction")
                        .interact_text()
                        .unwrap();
                    let amount: f64 = Input::new()
                        .with_prompt("Nouveau montant de la transaction")
                        .interact_text()
                        .unwrap();
                    budget.modifier_transaction(index - 1, description, amount);
                } else {
                    println!("Budget '{}' introuvable !", name);
                }
            }
            5 => {
                let name: String = Input::new()
                    .with_prompt("Nom du budget")
                    .interact_text()
                    .unwrap();

                if let Some(budget) = budgets.get_mut(&name) {
                    budget.afficher_transactions();
                    let index: usize = Input::new()
                        .with_prompt("Index de la transaction à supprimer")
                        .interact_text()
                        .unwrap();
                    budget.supprimer_transaction(index - 1);
                } else {
                    println!("Budget '{}' introuvable !", name);
                }
            }
            6 => {
                let name: String = Input::new()
                    .with_prompt("Nom du budget")
                    .interact_text()
                    .unwrap();

                if let Some(budget) = budgets.get(&name) {
                    budget.afficher_transactions();
                } else {
                    println!("Budget '{}' introuvable !", name);
                }
            }
            7 => {
                let name: String = Input::new()
                    .with_prompt("Nom du budget")
                    .interact_text()
                    .unwrap();

                if let Some(budget) = budgets.get(&name) {
                    let solde = budget.obtenir_solde();
                    println!("Solde pour le budget '{}': {:.2} €", name, solde);
                } else {
                    println!("Budget '{}' introuvable !", name);
                }
            }
            8 => {
                println!("Fermeture du gestionnaire de budgets. Au revoir !");
                break;
            }
            _ => unreachable!(),
        }
    }
}
