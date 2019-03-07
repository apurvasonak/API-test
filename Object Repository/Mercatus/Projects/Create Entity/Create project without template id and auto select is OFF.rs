<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Create project without template id and auto select is OFF</description>
   <name>Create project without template id and auto select is OFF</name>
   <tag></tag>
   <elementGuidId>3cdc0220-0509-4c00-a921-0d514db261cc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;autoSelectTemplateFlag\&quot;: false,\n\&quot;category\&quot;: \&quot;Project\&quot;,\n\&quot;organizationId\&quot;: \&quot;${OrgId}\&quot;,\n\&quot;shortName\&quot;: \&quot;Project without temp id autoselect is OFF\&quot;,\n\&quot;taskTemplateId\&quot;: null,\n\&quot;tzOffsetPlanned\&quot;: 330,\n\&quot;projectFieldMap\&quot;: {\n\&quot;25034\&quot;: \&quot;Test\&quot;,\n\&quot;25036\&quot;: \&quot;1\&quot;,\n\&quot;25038\&quot;: \&quot;a\&quot;,\n\&quot;25040\&quot;: \&quot;3218\&quot;,\n\&quot;25041\&quot;: \&quot;12/31/2018\&quot;,\n\&quot;25042\&quot;: \&quot;Yes\&quot;,\n\&quot;25056\&quot;: \&quot;Project\&quot;,\n\&quot;26376\&quot;: \&quot;1\&quot;\n}\n}&quot;,
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
      <id>84afbb29-7e88-4492-8499-046398c67363</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>findTestData('Create Entity').getValue(2, 1)</defaultValue>
      <description></description>
      <id>2d08539d-06b2-49e2-abff-8e194d4eb5c2</id>
      <masked>false</masked>
      <name>OrgId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Authorization</defaultValue>
      <description></description>
      <id>b5bf219b-7980-4192-b515-1867b1556697</id>
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
