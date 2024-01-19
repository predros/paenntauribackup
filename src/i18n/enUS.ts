import { en } from "vuetify/locale";

export default {
  $vuetify: {
    ...en,
    dataIterator: {
      rowsPerPageText: "Items per page:",
      pageText: "{0}-{1} of {2}",
    },
  },
  alerts: {
    invalidLoadcaseId: "Loadcase not found: id {0}",
    invalidMaterialId: "Material not found: id {0}",
    invalidMemberId: "Member not found: id {0}",
    invalidNodeId: "Node not found: id {0}",
    invalidSectionId: "Section not found: id {0}",
    materialInUse:
      "Unable to delete a material that is in use by one or more members.",
    nameInUseMaterial: "There is already a material with that name.",
    nameInUseSection: "There is already a section with that name.",
    nodeAlreadyExists:
      "There is already a node at the given coordinates (id {0}).",
    noMaterialSelected: "Please select a material before creating a member.",
    noSectionSelected: "Please select a section before creating a member.",
    sectionInUse:
      "Unable to delete a section that is in use by one or more members.",
    unstableStructure: "Failed to run analysis: unstable structure!",
  },
  app: {
    showGrid: "Show grid",
    snapToGrid: "Snap to grid",
    zoomIn: "Zoom in",
    zoomOut: "Zoom out",
  },
  buttons: {
    applyToSelection: "Apply to selection",
    cancel: "Cancel",
    clear: "Clear",
    close: "Close",
    delete: "Delete",
    edit: "Edit",
    manageMaterials: "Manage materials",
    manageSections: "Manage sections",
    newMaterial: "New material",
    save: "Save",
  },
  dialogs: {
    materials: {
      areYouSure:
        "Are you sure you want to delete material <b>{0}</b>? This action cannot be undone.",
      deleting: "Deleting material: {0}",
      elasticity: "Elasticity",
      name: "Name",
      thermal: "Thermal coeff.",
      title: "Manage materials",
      updating: "Editing material: {0}",
    },
  },
  errors: {
    distinctPoints: "The member's start and end points must be distinct.",
    ERROR: "ERROR",
    nonNegative: "Please input a non-negative value.",
    isPositive: "Please input a positive value.",
    uniqueMember: "There is already a member at the given coordinates.",
    uniqueNode: "There is already a node at the given coordinates.",
    validMaterial: "Please select a valid material for the member.",
    validNumber: "Please input a valid value.",
    validSection: "Please select a valid section for the member.",
  },
  menu: {
    analysis: {
      self: "Analysis",
    },
    edit: {
      self: "Edit",
    },
    file: {
      self: "File",
    },
    help: {
      self: "Help",
    },
    structure: {
      self: "Structure",
    },
  },
  sidebars: {
    forces: {
      forceX: "Force X",
      forceY: "Force Y",
      forceZ: "Moment",
      forceAngle: "Angle",
      title: "Nodal forces",
    },
    hinges: {
      memberEnds: "Hinge member ends",
      memberStarts: "Hinge member starts",
      nodes: "Hinge nodes",
      title: "Hinges",
    },
    loads: {
      isLinear: "Linear load",
      title: "Distributed loads",
      xEnd: "X end",
      xStart: "X start",
      yEnd: "Y end",
      yStart: "Y start",
    },
    matSec: {
      material: "Material",
      section: "Section",
      title: "Materials/sections",
    },
    member: {
      material: "Material",
      section: "Section",
      title: "New member",
      xEnd: "X end",
      xStart: "X start",
      yEnd: "Y end",
      yStart: "Y start",
    },
    node: {
      title: "New node",
      x: "X coord.",
      y: "Y coord.",
    },
    supports: {
      displacementX: "X direction",
      displacementY: "Y direction",
      displacementZ: "Rotation",

      labelDisplacements: "Displac.",
      labelSprings: "Springs",
      labelSupports: "Supports",

      springX: "X direction",
      springY: "Y direction",
      springZ: "Rotation",

      supportAngle: "Angle",
      supportX: "X displacement",
      supportY: "Y displacement",
      supportZ: "Rotation",

      title: "Supports and springs",
      titleDisplacements: "Prescribed displacements",
      titleSprings: "Spring constants",
      titleSupports: "Supports",
    },
    temperature: {
      inferior: "Inferior variation",
      superior: "Superior variation",
      title: "Temperature variations",
    },
  },
  toolbars: {
    side: {
      forces: "Nodal forces",
      hinges: "Hinges",
      loads: "Distributed loads",
      matSec: "Materials and sections",
      member: "New member",
      node: "New node",
      select: "Select",
      supports: "Supports and springs",
      temperature: "Temperature variations",
    },
    top: {
      loadcaseCurrent: "Loadcase",
      loadcases: "Loadcases and combinations",
      new: "New project",
      open: "Open",
      redo: "Redo",
      run: "Run analysis",
      save: "Save",
      saveAs: "Save as...",
      settings: "Settings",
      undo: "Undo",
    },
  },
};
