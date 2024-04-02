package de.provadis.brunne.wab1.service.datamodel;

import java.time.OffsetDateTime;
import java.util.List;
import java.util.Objects;
import java.util.UUID;

public final class Order {
    private final String id;
    private final OffsetDateTime orderedOn;
    private final List<Product> products;
    private OrderStatus status;

    public Order(String id,
                 OffsetDateTime orderedOn,
                 List<Product> products,
                 OrderStatus status) {
        this.id = id;
        this.orderedOn = orderedOn;
        this.products = products;
        this.status = status;
    }
    public Order(String id,
                 List<Product> products) {
        this(id, OffsetDateTime.now(), products, OrderStatus.PENDING);
    }

    public String getId() {
        return id;
    }

    public OffsetDateTime getOrderedOn() {
        return orderedOn;
    }

    public List<Product> getProducts() {
        return products;
    }

    public OrderStatus getStatus() {
        return status;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == this) return true;
        if (obj == null || obj.getClass() != this.getClass()) return false;
        var that = (Order) obj;
        return Objects.equals(this.id, that.id) &&
                Objects.equals(this.orderedOn, that.orderedOn) &&
                Objects.equals(this.products, that.products) &&
                Objects.equals(this.status, that.status);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, orderedOn, products, status);
    }

    @Override
    public String toString() {
        return "Order[" +
                "id=" + id + ", " +
                "orderedOn=" + orderedOn + ", " +
                "products=" + products + ", " +
                "status=" + status + ']';
    }
}
