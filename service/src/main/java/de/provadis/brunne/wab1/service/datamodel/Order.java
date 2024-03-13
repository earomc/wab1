package de.provadis.brunne.wab1.service.datamodel;

import java.time.OffsetDateTime;
import java.util.UUID;

public record Order(UUID id, OffsetDateTime orderedOn, UUID customerId) {}
