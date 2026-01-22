/// Unit test: Initialize Priority Graph Data Structure
/// 
/// Analogy: Think of a priority graph like a restaurant reservation system:
/// - Nodes (transactions) = Customers with priority levels (VIP, regular, etc.)
/// - Edges (accounts) = Shared tables/resources that create conflicts
/// - Graph = Tracks which customers want the same tables
/// 
/// This test demonstrates how to create and initialize a priority graph,
/// which is the foundation for scheduling transactions in Solana.

#[cfg(test)]
mod tests {
    use prio_graph::{AccessKind, GraphNode, PrioGraph, TopLevelId};
    use solana_pubkey::Pubkey;

    // Define a simple priority ID type (like TransactionPriorityId)
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct SimplePriorityId {
        priority: u64,
        id: usize,
    }

    impl SimplePriorityId {
        fn new(priority: u64, id: usize) -> Self {
            Self { priority, id }
        }
    }

    // Implement TopLevelId trait required by PrioGraph
    impl TopLevelId<Self> for SimplePriorityId {
        fn id(&self) -> Self {
            *self
        }
    }

    // Priority function: returns the priority ID as-is
    // Analogy: Like a restaurant host who uses the customer's VIP level directly
    fn passthrough_priority(
        id: &SimplePriorityId,
        _graph_node: &GraphNode<SimplePriorityId>,
    ) -> SimplePriorityId {
        *id
    }

    // Define the priority graph type
    // Analogy: This is like defining the structure of our reservation system
    // - SimplePriorityId: The customer (node)
    // - Pubkey: The table/account they want (edge label)
    // - SimplePriorityId: The value stored in the graph
    // - passthrough_priority: How we determine priority
    type TestPrioGraph = PrioGraph<
        SimplePriorityId,  // Node type
        Pubkey,             // Edge type (account/resource)
        SimplePriorityId,  // Value type
        fn(&SimplePriorityId, &GraphNode<SimplePriorityId>) -> SimplePriorityId,  // Priority function
    >;

    #[test]
    fn test_priority_graph_initialization() {
        // Step 1: Create a new priority graph
        // Analogy: Opening a new reservation book at the restaurant
        let mut prio_graph: TestPrioGraph = PrioGraph::new(passthrough_priority);

        // Step 2: Verify the graph is empty initially
        // Analogy: The reservation book starts empty
        assert!(prio_graph.is_empty(), "Graph should be empty when first created");

        // Step 3: Create some test accounts (like table numbers)
        // Analogy: These are the tables/resources customers want
        let account_a = Pubkey::new_unique();
        let account_b = Pubkey::new_unique();

        // Step 4: Create priority IDs (like customer VIP levels)
        // Analogy: Customer 1 has priority 100 (VIP), Customer 2 has priority 50 (regular)
        let tx1_id = SimplePriorityId::new(100, 1);
        let tx2_id = SimplePriorityId::new(50, 2);

        // Step 5: Insert transactions into the graph
        // Analogy: Adding customers to the reservation system with their desired tables
        // Transaction 1 wants account_a (read access)
        prio_graph.insert_transaction(
            tx1_id,
            std::iter::once((account_a, AccessKind::Read)),
        );

        // Verify graph is no longer empty
        assert!(!prio_graph.is_empty(), "Graph should not be empty after inserting a transaction");

        // Transaction 2 wants account_b (write access)
        prio_graph.insert_transaction(
            tx2_id,
            std::iter::once((account_b, AccessKind::Write)),
        );

        // Step 6: Verify we can pop transactions in priority order
        // Analogy: The host calls customers in VIP order (highest priority first)
        let popped_first = prio_graph.pop();
        assert_eq!(
            popped_first,
            Some(tx1_id),
            "Should pop highest priority transaction first (priority 100)"
        );

        let popped_second = prio_graph.pop();
        assert_eq!(
            popped_second,
            Some(tx2_id),
            "Should pop next highest priority transaction (priority 50)"
        );

        // Step 7: Verify graph is empty after popping all transactions
        assert!(prio_graph.is_empty(), "Graph should be empty after popping all transactions");
    }
}
