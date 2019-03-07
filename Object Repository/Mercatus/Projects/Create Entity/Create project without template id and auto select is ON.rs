<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create project without template id and auto select is ON</description>
   <name>Create project without template id and auto select is ON</name>
   <tag></tag>
   <elementGuidId>a3d80788-8b2b-4553-9c27-3f30f81a6817</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;autoSelectTemplateFlag\&quot;: true,\n\&quot;category\&quot;: \&quot;Project\&quot;,\n\&quot;organizationId\&quot;: \&quot;${OrgId}\&quot;,\n\&quot;shortName\&quot;: \&quot;Project with template Auto select ON\&quot;,\n\&quot;taskTemplateId\&quot;: null,\n\&quot;tzOffsetPlanned\&quot;: 330,\n\&quot;projectFieldMap\&quot;: {\n\&quot;25034\&quot;: \&quot;fd\&quot;,\n\&quot;25036\&quot;: \&quot;1\&quot;,\n\&quot;25038\&quot;: \&quot;a\&quot;,\n\&quot;25040\&quot;: \&quot;3218\&quot;,\n\&quot;25041\&quot;: \&quot;12/31/2018\&quot;,\n\&quot;25042\&quot;: \&quot;Yes\&quot;,\n\&quot;25056\&quot;: \&quot;Project\&quot;,\n\&quot;26376\&quot;: \&quot;1\&quot;\n}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${Authorization}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/rest/projects?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(1, 1)</defaultValue>
      <description></description>
      <id>dd920bc5-b490-4b58-9ff4-2153b2e380d1</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(2, 1)</defaultValue>
      <description></description>
      <id>841b6619-9237-4a48-b76e-dae769d0606a</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>32d07748-a017-48f3-a1ce-3fcbba42fe70</id>
      <masked>false</masked>
      <name>Authorization</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
